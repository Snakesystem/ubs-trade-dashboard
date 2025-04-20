<script>
  import { onMount } from "svelte";
  
  let columns = [
    { field: "AppComputerName", title: "AppComputerName" },
    { field: "AppIPAddress", title: "AppIPAddress" },
    { field: "CIFLookupDescription", title: "CIFLookupDescription" },
    { field: "CIFLookupID", title: "CIFLookupID" },
    { field: "CIFLookupInteger", title: "CIFLookupInteger" },
    { field: "CIFLookupNID", title: "CIFLookupNID" },
    { field: "CIFLookupString", title: "CIFLookupString" },
    { field: "ChangeNID", title: "ChangeNID" },
    { field: "ColumnName", title: "ColumnName" },
    { field: "Description", title: "Description" },
    { field: "EndIncome", title: "EndIncome" },
    { field: "LastUpdate", title: "LastUpdate" },
    { field: "MaxNetBuyPercent", title: "MaxNetBuyPercent" },
    { field: "Risk", title: "Risk" },
    { field: "SBROccupationCode", title: "SBROccupationCode" },
    { field: "StartIncome", title: "StartIncome" },
    { field: "SysTerminalID", title: "SysTerminalID" },
    { field: "SysUserID", title: "SysUserID" },
    { field: "TableName", title: "TableName" },
    { field: "TimeStamp", title: "TimeStamp" },
  ];

  onMount(() => {
    // @ts-ignore
    globalThis.$("#myTable").bootstrapTable({
      url: "http://localhost:8000/v1/data/get-table?tablename=CIFLookup&nidkey=CIFLookupNID",
      method: "GET",
      contentType: "application/json",
      dataType: "json",
      cache: true,
      sidePagination: "server",
      pagination: true,
      showRefresh: true,
      showColumns: true,
      toolbar:".toolbar",
      offset: 0,
      pageSize: 100,
      pageList: [100, 200, 500],
      height: 750,
      columns: columns,
    });
  });
</script>

<section>
  <!-- Tabs (di bawah) -->
  <ul class="nav nav-tabs bottom-tab" id="myTab" role="tablist">
    <li class="nav-item" role="presentation">
      <button class="nav-link active" id="data-tab" data-bs-toggle="tab" data-bs-target="#data-tab-pane" type="button" role="tab" aria-controls="data-tab-pane" aria-selected="false">Data Table</button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="bar-chart-tab" data-bs-toggle="tab" data-bs-target="#bar-chart-tab-pane" type="button" role="tab" aria-controls="bar-chart-tab-pane" aria-selected="false">Bar Chart</button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="line-chart-tab" data-bs-toggle="tab" data-bs-target="#line-chart-tab-pane" type="button" role="tab" aria-controls="line-chart-tab-pane" aria-selected="false">Line Chart</button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="pie-chart-tab" data-bs-toggle="tab" data-bs-target="#pie-chart-tab-pane" type="button" role="tab" aria-controls="pie-chart-tab-pane" aria-selected="false">Pie Chart</button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="scatter-chart-tab" data-bs-toggle="tab" data-bs-target="#scatter-chart-tab-pane" type="button" role="tab" aria-controls="scatter-chart-tab-pane" aria-selected="false">Scatter Chart</button>
    </li>
    <li class="nav-item" role="presentation">
      <button class="nav-link" id="radar-chart-tab" data-bs-toggle="tab" data-bs-target="#radar-chart-tab-pane" type="button" role="tab" aria-controls="radar-chart-tab-pane" aria-selected="false">Radar Chart</button>
    </li>
  </ul>

  <!-- Tab Content (di atas) -->
  <div class="tab-content" id="myTabContent">
    <div class="tab-pane fade show active" id="data-tab-pane" role="tabpanel" aria-labelledby="data-tab" tabindex="0" >
      <!-- HTML Table -->
      <div class="toolbar">
        <p>Filter</p>
      </div>
      <table id="myTable" class="table table-striped" data-toggle="table"></table>
    </div>
    <div class="tab-pane fade" id="bar-chart-tab-pane" role="tabpanel" aria-labelledby="bar-chart-tab" tabindex="0" >
      <!-- HTML Chart -->
      Bar Chart
    </div>
    <div class="tab-pane fade" id="line-chart-tab-pane" role="tabpanel" aria-labelledby="line-chart-tab" tabindex="0" >
      <!-- HTML Chart -->
      Line Chart
    </div>
    <div class="tab-pane fade" id="pie-chart-tab-pane" role="tabpanel" aria-labelledby="pie-chart-tab" tabindex="0" >
      <!-- HTML Chart -->
      Pie Chart
    </div>
    <div class="tab-pane fade" id="scatter-chart-tab-pane" role="tabpanel" aria-labelledby="scatter-chart-tab" tabindex="0" >
      <!-- HTML Chart -->
      Scatter Chart
    </div>
    <div class="tab-pane fade" id="radar-chart-tab-pane" role="tabpanel" aria-labelledby="radar-chart-tab" tabindex="0" >
      <!-- HTML Chart -->
      Radar Chart
    </div>
  </div>
</section>
