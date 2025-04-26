<script>
  // @ts-nocheck
  import { onMount, tick } from "svelte";
  import Chart from "chart.js/auto";
  import Sortable from "sortablejs";
  import { base_url, openModal } from "../app";
  import ModalContainer from "./ModalContainer.svelte";
  import{ location } from 'svelte-spa-router';
  import Swal from "sweetalert2";

  function getRandomColor() {
    const hue = Math.floor(Math.random() * 360);
    return `hsl(${hue}, 70%, 60%)`;
  }

  function convertToChart(data) {
    return data.map((item) => {
      const chartId = item.ChartID; // Ganti dengan ChartID dari API
      const chartTitle = item.ChartName; // Ganti dengan ChartName dari API

      const labels = item.data.data.map((entry) => entry.Value);
      const values = item.data.data.map((entry) => entry.Count);
      const color = getRandomColor();

      return {
        id: chartId,
        title: chartTitle,
        type: "bar",
        data: {
          labels,
          datasets: [
            {
              label: chartTitle, // Gunakan ChartName sebagai label dataset
              data: values,
              backgroundColor: color,
            },
          ],
        },
      };
    });
  }

  let sortableContainer;
  let charts = [];
  let chart_data = [];

  export let filter;
  export let tablename;
  export let column = [];
  
  let formData = {
    chart_name: "",
    menu_id: $location.replace("/",""),
    list_column: "",
  }

  let deleteData = {
    chart_id: "",
    menu_id: $location.replace("/",""),
  }

  let updateData = {
    chart_id: "",
  }

  async function getChartData(menu_id) {
    const response = await fetch(`${base_url}/chart/data/${menu_id}`);
    return await response.json();
  }

  async function getBarChart(column) {
    const response = await fetch(
      `${base_url}/chart/bar?filter=${encodeURIComponent(JSON.stringify(filter))}&tablename=${tablename}&column=${column}`
    );
    return await response.json();
  }

  async function initCharts() {
    const chartData = await getChartData("home");
    chart_data = chartData.data;
    let barDataList = [];

    if (chartData.result && Array.isArray(chartData.data)) {
      const columns = chartData.data.map((item) => item.ListColumn);

      for (const col of columns) {
        const data = await getBarChart(col);
        // Menambahkan ID dan Name untuk setiap chart
        const chartDataWithInfo = chartData.data.find(
          (item) => item.ListColumn === col
        );
        barDataList.push({
          column: col,
          data,
          ChartID: chartDataWithInfo.ChartID,
          ChartName: chartDataWithInfo.ChartName,
        });
      }

      charts = convertToChart(barDataList);

      // Tunggu sampai DOM selesai render canvas
      await tick();

      // Render chart-nya setelah DOM siap
      charts.forEach((chart) => {
        const ctx = document.getElementById(chart.id);
        if (ctx) {
          new Chart(ctx, {
            type: chart.type,
            data: chart.data,
            options: {
              responsive: true,
              maintainAspectRatio: false,
            },
          });
        }
      });

      // Inisialisasi drag
      Sortable.create(sortableContainer, {
        animation: 200,
        ghostClass: "drag-ghost",
      });
    }
  }

  onMount(() => {
    initCharts();
  });

  async function submit(change) {
    const response = await fetch(`${base_url}/chart/${change}-bar`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(formData),
    });
    const result = await response.json();
    if (result.result) {
      await Swal.fire({
        icon: "success",
        title: result.message,
        text: "Chart has been saved",
      })
      await initCharts();
    } else {
      await Swal.fire({
        icon: "error",
        title: result.message,
        text: result.error,
      })
    }
  }

  async function deleted() {
    const response = await fetch(`${base_url}/chart/delete-bar`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(deleteData),
    });
    const result = await response.json();
    if (result.result) {
      await Swal.fire({
        icon: "success",
        title: result.message,
        text: "Chart has been deleted",
      })
      await initCharts();
    } else {
      await Swal.fire({
        icon: "error",
        title: result.message,
        text: result.error,
      })
    }
  }

</script>

<button aria-label="filter" class="btn btn-success my-3" type="button" onclick={() => openModal("add-bar")}><i class="bi bi-cart-plus"></i></button>
<button aria-label="filter" class="btn btn-info my-3" type="button" onclick={() => openModal("update-bar")}><i class="bi bi-cart-check"></i></button>
<button aria-label="filter" class="btn btn-danger my-3" type="button" onclick={() => openModal("delete-bar")}><i class="bi bi-trash"></i></button>
<div bind:this={sortableContainer} class="dashboard-container">
  {#each charts as chart}
    <div class="card-item">
      <h6 class="fw-bold text-center mb-2">{chart.title}</h6>
      <canvas id={chart.id}></canvas>
    </div>
  {/each}
</div>
<ModalContainer id="add-bar" title="Add Bar Chart" size="sm" >
    <form onsubmit={async(e) => {
        e.preventDefault();
        await submit("create");
    }} >
        <div class="row">
            <div class="col-md-12 mb-3">
                <label for="chart_name" class="form-label">Chart Name</label>
                <input type="text" required class="form-control form-control-sm" id="chart_name" bind:value={formData.chart_name} placeholder="Chart Name">
            </div>
            <div class="col-md-12 mb-3">
                <label for="menu_id" class="form-label">Menu ID</label>
                <input type="text" disabled required class="form-control form-control-sm" id="menu_id" bind:value={formData.menu_id} placeholder="Menu ID">
            </div>
            <div class="col-md-12 mb-3">
                <label for="list_column" class="form-label">List Column</label>
                <select name="list_column" id="list_column" class="form-select form-select-sm" bind:value={formData.list_column} required>
                    <option value="">Select Column</option>
                    {#each column as col}
                        <option value={col.field}>{col.title}</option>
                    {/each}
                </select>
            </div>
            <div class="col-md-12 mb-3">
                <button type="submit" class="btn btn-primary btn-sm">Submit</button>
            </div>
        </div>
    </form>
</ModalContainer>
<ModalContainer id="update-bar" title="Update Bar Chart" size="sm" >
  <form onsubmit={async(e) => {
      e.preventDefault();
      await submit("update");
    }} >
      <div class="row">
        <div class="col-md-12 mb-3">
          <label for="list_column" class="form-label">Charts</label>
            <select name="list_column" id="list_column" 
                    class="form-select form-select-sm" 
                    bind:value={updateData.chart_id} 
                    onchange={(e) => {
                      let update_data = chart_data.find(item => item.ChartID == e.target.value);
                      formData.chart_name = update_data.ChartName;
                      formData.menu_id = update_data.MenuID;
                      formData.list_column = update_data.ListColumn;
                    }}
                    required>
                <option value="">Select Chart</option>
                {#each chart_data as chart}
                    <option value={chart.ChartID}>{chart.ChartName}</option>
                {/each}
            </select>
        </div>
      </div>
      {#if updateData.chart_id}
        <div class="row">
          <div class="col-md-12 mb-3">
              <label for="chart_name" class="form-label">Chart Name</label>
              <input type="text" required class="form-control form-control-sm" id="chart_name" bind:value={formData.chart_name} placeholder="Chart Name">
          </div>
          <div class="col-md-12 mb-3">
              <label for="menu_id" class="form-label">Menu ID</label>
              <input type="text" disabled required class="form-control form-control-sm" id="menu_id" bind:value={formData.menu_id} placeholder="Menu ID">
          </div>
          <div class="col-md-12 mb-3">
              <label for="list_column" class="form-label">List Column</label>
              <select name="list_column" id="list_column" class="form-select form-select-sm" bind:value={formData.list_column} required>
                  <option value="">Select Column</option>
                  {#each column as col}
                      <option value={col.field}>{col.title}</option>
                  {/each}
              </select>
          </div>
          <div class="col-md-12 mb-3">
              <button type="submit" class="btn btn-primary btn-sm">Submit</button>
          </div>
        </div>
      {/if}
  </form>
</ModalContainer>
<ModalContainer id="delete-bar" title="Delete Bar Chart" size="sm" >
  <form onsubmit={async(e) => {
      e.preventDefault();
      await deleted();
  }} >
      <div class="row">
          <div class="col-md-12 mb-3">
              <label for="list_column" class="form-label">Charts</label>
              <select name="list_column" id="list_column" class="form-select form-select-sm" bind:value={deleteData.chart_id} required>
                  <option value="">Select Column</option>
                  {#each chart_data as chart}
                      <option value={chart.ChartID}>{chart.ChartName}</option>
                  {/each}
              </select>
          </div>
          <div class="col-md-12 mb-3">
              <button type="submit" class="btn btn-danger btn-sm w-100">Delete</button>
          </div>
      </div>
  </form>
</ModalContainer>

<!-- svelte-ignore css_unused_selector -->
<style>
  .dashboard-container {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .card-item {
    flex: 1 1 calc(33.333% - 1rem);
    background: white;
    padding: 1rem;
    border-radius: 8px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
    cursor: move;
  }

  canvas {
    width: 100% !important;
    height: 240px !important;
  }

  .drag-ghost {
    opacity: 0.4;
  }

  @media (max-width: 768px) {
    .card-item {
      flex: 1 1 100%;
    }
  }
</style>
