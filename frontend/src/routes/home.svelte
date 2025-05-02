<script>
  import { onMount } from "svelte";
  import BarChart from "../lib/BarChart.svelte";
  import { base_url, fetchColumns, openModal } from "../app";
  import ModalContainer from "../lib/ModalContainer.svelte";
  import ModalFilter from "../lib/ModalFilter.svelte";

  let tablename = "CIFLookup";
  let tablePK = "CIFLookupNID";

  let filter = $state({});

  onMount(() => { 
    if(localStorage.getItem(`${tablename}_filter`)) {
      filter = JSON.parse(localStorage.getItem(`${tablename}_filter`));
    } else {
      localStorage.setItem(`${tablename}_filter`, JSON.stringify(filter));
    }
   })

  let result = $state({
    result: false,
    message: "",
    data: [],
    error: ""
  });

  function updateFilter(data) {
    filter = data;
    localStorage.setItem(`${tablename}_filter`, JSON.stringify(data));
  }

  function toolbarButton () {
    return {
      btnFilter: {
        text: "Filter",
        icon: "bi bi-search",
        event: function () {
          openModal("filter");
        },
        attributes: {
          title: "Search all data",
        }
      }
    }
  }

  function initTable(columns) {
    // Destroy table first if already exists
    // @ts-ignore
    globalThis.$("#myTable").bootstrapTable("destroy");

    // Initialize new table
    // @ts-ignore
    globalThis.$("#myTable").bootstrapTable({
      url: `${base_url}/data/get-table?tablename=${tablename}&nidkey=${tablePK}&filter=${JSON.stringify(filter)}`,
      method: "GET",
      contentType: "application/json",
      buttons: toolbarButton(),
      buttonsClass: "primary",
      // buttonsAlign: "left",
      // toolbarAlign: "right",
      dataType: "json",
      cache: true,
      sidePagination: "server",
      showColumns: true,
      pagination: true,
      showRefresh: true,
      autoRefresh: true,
      autoRefreshInterval: 120,
      toolbar: ".toolbar",
      offset: 0,
      pageSize: 100,
      pageList: [100, 200, 500],
      height: 750,
      columns: columns
    });
  }

  onMount(async () => {
    result = await fetchColumns(tablename);
    if (result.result && result.data && Array.isArray(result.data)) {
      initTable(result.data);
    } else {
      console.error("Data columns not valid", result);
    }
  });

</script>

<section>
  <!-- Tabs -->
  <ul class="nav nav-tabs bottom-tab" id="myTab" role="tablist">
    <li class="nav-item" role="presentation">
      <button class="nav-link active" id="data-tab" data-bs-toggle="tab" data-bs-target="#data-tab-pane" type="button" role="tab" aria-controls="data-tab-pane" aria-selected="true">Data Table</button>
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

  <!-- Tab Content -->
  <div class="tab-content" id="myTabContent">
    <div class="tab-pane fade show active" id="data-tab-pane" role="tabpanel" aria-labelledby="data-tab">
      <div class="toolbar">
        {#if Object.entries(filter).length > 0}
          FIlter: 
          <span class="text-muted">
            {#each Object.entries(filter) as [key, value], index}
              {#if key === 'IsRejected' || key === 'IsRevised' || key === 'IsFinished'}
                {key}: {value === '0' ? 'No' : 'Yes'}
              {:else}
                {key}: {value}
              {/if}
              {#if index < Object.entries(filter).length - 1}
                {' | '}
              {/if}
            {/each}
          </span>
        {:else}
          <span class="text-muted fw-bold">No Filter</span>
        {/if}
      </div>
      <table id="myTable" class="table table-striped" data-toggle="table"></table>
    </div>
    <div class="tab-pane fade" id="bar-chart-tab-pane" role="tabpanel" aria-labelledby="bar-chart-tab">
      <button aria-label="filter" class="btn btn-primary my-3" type="button" onclick={() => openModal('filter')}><i class="bi bi-search"></i></button>
      <BarChart tablename={tablename} filter={filter} column={result.data}/>
    </div>
    <div class="tab-pane fade" id="line-chart-tab-pane" role="tabpanel" aria-labelledby="line-chart-tab">Line Chart</div>
    <div class="tab-pane fade" id="pie-chart-tab-pane" role="tabpanel" aria-labelledby="pie-chart-tab">Pie Chart</div>
    <div class="tab-pane fade" id="scatter-chart-tab-pane" role="tabpanel" aria-labelledby="scatter-chart-tab">Scatter Chart</div>
    <div class="tab-pane fade" id="radar-chart-tab-pane" role="tabpanel" aria-labelledby="radar-chart-tab">Radar Chart</div>
  </div>
  <ModalContainer id="filter" title="Filter Data Table" size="lg" >
    <ModalFilter data={result.data} tablename={tablename} updateFilter={updateFilter} initTable={initTable} />
  </ModalContainer>
</section>

<style scoped>
  section {
      padding: 1rem 1rem 2rem 1rem;
      max-height: 90vh;
      overflow-y: auto;
  }
</style>