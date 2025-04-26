use std::collections::HashMap;

use actix_web::web;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use crate::contexts::{connection::Transaction, model::{ActionResult, BarChartParams, BarChartRequest, DeleteBarChart}};
use super::data_service::DataService;

pub struct ChartService;

impl ChartService {
    pub async fn get_chart_data(connection: web::Data<bb8::Pool<ConnectionManager>>, menu_id: String) -> ActionResult<Vec<serde_json::Value>, String> {
        let mut result: ActionResult<Vec<serde_json::Value>, String> = ActionResult::default();
    
        match connection.clone().get().await {
            Ok(mut conn) => {
                let menu_id_param = format!("{}", menu_id);
    
                let query = r#"SELECT * FROM [BarChart] WHERE MenuID = @P1"#;
    
                match conn.query(query, &[&menu_id_param]).await {
                    Ok(rows) => {      
    
                        let data = rows.into_results().await.unwrap();

                        let row_data = data.into_iter()
                        .flat_map(|r| r.into_iter())
                        .map(|row| DataService::row_to_json(&row));
                        
                        result.data = Some(row_data.collect());
                    
                        result.result = true;
                        result.message = "Data retrieved successfully".to_string();
                        return result;
                    }
                    Err(e) => {
                        result.message = "Internal Server Error".to_string();
                        result.error = Some(e.to_string());
                        return result;
                    }
                }
            }
            Err(e) => {
                result.message = "Database connection failed".to_string();
                result.error = Some(e.to_string());
                return result;
            }
        }
    }    

    // #region BAR CHART SERVICE
    pub async fn get_bar_chart(connection: web::Data<bb8::Pool<ConnectionManager>>, params: BarChartParams) -> ActionResult<Vec<serde_json::Value>, String> {
        let mut result: ActionResult<Vec<serde_json::Value>, String> = ActionResult::default();
        let mut q_and_where = String::from(" WHERE 1=1 ");

        match connection.clone().get().await {
            
            Ok(mut conn) => {
                if let Some(filter) = &params.filter {
                    // println!("Filter: {}", filter);
                    if filter != "{filter:undefined}" {
                        match serde_json::from_str::<HashMap<String, String>>(filter) {
                            Ok(filter_name) => {
                                if !filter_name.is_empty() {
                                    let q_and_where_result = DataService::get_query_table_where(q_and_where.clone(), filter_name);
            
                                    q_and_where = q_and_where_result.clone();
                                }
                            },
                            Err(e) => {
                                result.message = "Invalid filter".to_string();
                                result.error = Some(e.to_string());
                                return result;
                            }
                        }
                    }
                }
    
                let query = format!(
                    "SELECT {} AS Value, COUNT(*) as Count FROM {} {} GROUP BY {}",
                    &params.column, &params.tablename, &q_and_where, &params.column
                );
                
                // println!("Query: {}, filter: {}", query, params.filter.unwrap());
    
                match conn.query(query, &[]).await {
                    Ok(rows) => {      
    
                        let data = rows.into_results().await.unwrap();

                        let row_data = data.into_iter()
                        .flat_map(|r| r.into_iter())
                        .map(|row| DataService::row_to_json(&row));
                        
                        result.data = Some(row_data.collect());
                    
                        result.result = true;
                        result.message = "Data retrieved successfully".to_string();
                        return result;
                    }
                    Err(e) => {
                        result.message = "Internal Server Error".to_string();
                        result.error = Some(e.to_string());
                        return result;
                    }
                }
            }
            Err(e) => {
                result.message = "Database connection failed".to_string();
                result.error = Some(e.to_string());
                return result;
            }
        }
    }

    pub async fn save_bar_chart(connection: web::Data<Pool<ConnectionManager>>, request: BarChartRequest) -> ActionResult<(), String> {
        let mut result: ActionResult<(), String> = ActionResult::default();
            
        match Transaction::begin(&connection).await {
            Ok(trans) => {

                let joined_list_column = request.list_column;
                let chart_id = request.chart_name.clone().unwrap_or_default().replace(" ", "-").to_lowercase();

                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        
                        match conn.execute(
                            r#"INSERT INTO [dbo].[BarChart] 
                            ([ChartID],[ChartName],[MenuID],[ListColumn])
                            VALUES
                            (@P1,@P2,@P3,@P4)"#,
                            &[
                                &chart_id, &request.chart_name, &request.menu_id, &joined_list_column
                            ],
                        ).await {
                            Ok(query_result) => {
                                let rows = query_result.rows_affected();
                                if rows.iter().any(|row| row > &0) {
                                    result.result = true;
                                    result.message = format!("{} BarChart saved successfully", rows.iter().count());
                                } else {
                                    result.result = false;
                                    result.message = "No BarChart found to save".to_string();
                                }

                            }
                            Err(err) => {
                                result.message = "Failed".to_string();
                                result.error = Some(format!("Failed to delete BarChart: {}", err));
                                return result;
                            }
                        }                        
                    }
                    None => {
                        result.error = Some("Failed to get connection from pool".into());
                        return result;
                    }
                }
    
                if let Err(err) = trans.commit().await {
                    result.error = Some(format!("Failed to commit transaction: {:?}", err));
                    return result;
                }
            }
            Err(err) => {
                result.error = Some(format!("Failed to start transaction: {:?}", err));
            }
        }
    
        result
    }

    pub async fn update_bar_chart(connection: web::Data<Pool<ConnectionManager>>, request: BarChartRequest) -> ActionResult<(), String> {
        let mut result: ActionResult<(), String> = ActionResult::default();
            
        match Transaction::begin(&connection).await {
            Ok(trans) => {

                let joined_list_column = request.list_column;
                let chart_id = request.chart_name.clone().unwrap_or_default().replace(" ", "-").to_lowercase();

                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        
                        match conn.execute(
                            r#"UPDATE [dbo].[BarChart] SET 
                            [ChartName] = @P2, [MenuID] = @P3, [ListColumn] = @P4 WHERE [ChartID] = @P1"#,
                            &[
                                &chart_id, &request.chart_name, &request.menu_id, &joined_list_column
                            ],
                        ).await {
                            Ok(query_result) => {
                                let rows = query_result.rows_affected();
                                if rows.iter().any(|row| row > &0) {
                                    result.result = true;
                                    result.message = format!("{} BarChart updated successfully", rows.iter().count());
                                } else {
                                    result.result = false;
                                    result.message = "No BarChart found to update".to_string();
                                }

                            }
                            Err(err) => {
                                result.message = "Failed".to_string();
                                result.error = Some(format!("Failed to delete BarChart: {}", err));
                                return result;
                            }
                        }                        
                    }
                    None => {
                        result.error = Some("Failed to get connection from pool".into());
                        return result;
                    }
                }
    
                if let Err(err) = trans.commit().await {
                    result.error = Some(format!("Failed to commit transaction: {:?}", err));
                    return result;
                }
            }
            Err(err) => {
                result.error = Some(format!("Failed to start transaction: {:?}", err));
            }
        }
    
        result
    }

    pub async fn delete_bar_chart(connection: web::Data<Pool<ConnectionManager>>, request: DeleteBarChart) -> ActionResult<(), String> {
        let mut result: ActionResult<(), String> = ActionResult::default();
            
        match Transaction::begin(&connection).await {
            Ok(trans) => {
                match trans.conn.lock().await.as_mut() {
                    Some(conn) => {
                        match conn.execute(
                            r#"DELETE FROM [dbo].[BarChart] WHERE ChartID = @P1 AND MenuID = @P2"#,
                            &[&request.chart_id, &request.menu_id],
                        ).await {
                            Ok(query_result) => {
                                let rows = query_result.rows_affected();
                                if rows.iter().any(|row| row > &0) {
                                    result.result = true;
                                    result.message = format!("{} BarChart(s) deleted successfully", rows.iter().count());
                                } else {
                                    result.result = false;
                                    result.message = "No BarChart found to delete".to_string();
                                }

                            }
                            Err(err) => {
                                result.message = "Failed".to_string();
                                result.error = Some(format!("Failed to delete BarChart: {}", err));
                                return result;
                            }
                        }                        
                    }
                    None => {
                        result.error = Some("Failed to get connection from pool".into());
                        return result;
                    }
                }
    
                if let Err(err) = trans.commit().await {
                    result.error = Some(format!("Failed to commit transaction: {:?}", err));
                    return result;
                }
            }
            Err(err) => {
                result.error = Some(format!("Failed to start transaction: {:?}", err));
            }
        }
    
        result
    }
    
    // #endregion

    // #region LINE CHART SERVICE
    pub async fn get_line_chart() -> String {
        "Line Chart".to_string()
    }

    pub async fn save_line_chart() -> String {
        "Line Chart".to_string()
    }
    // #endregion

    // #region PIE CHART SERVICE
    pub async fn get_pie_chart() -> String {
        "Pie Chart".to_string()
    }

    pub async fn save_pie_chart() -> String {
        "Pie Chart".to_string()
    }
    // #endregion

    // #region PIE CHART SERVICE
    pub async fn get_scatter_chart() -> String {
        "Scatter Chart".to_string()
    }

    pub async fn save_scatter_chart() -> String {
        "Scatter Chart".to_string()
    }
    // #endregion

    // #region RADAR CHART SERVICE
    pub async fn get_radar_chart() -> String {
        "Radar Chart".to_string()
    }

    pub async fn save_radar_chart() -> String {
        "Radar Chart".to_string()
    }
    // #endregion
}