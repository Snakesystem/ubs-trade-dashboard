<script>
    import { onMount } from "svelte";
    import ModalContainer from "./ModalContainer.svelte";
    import ModalFilter from "./ModalFilter.svelte";

    export let tablename;
    export let filter;
    export let data = [];
    export let initTable;

    onMount(() => { 
        if(localStorage.getItem(`${tablename}_filter`)) {
            filter = JSON.parse(localStorage.getItem(`${tablename}_filter`));
        } else {
            localStorage.setItem(`${tablename}_filter`, JSON.stringify(filter));
        }
    });

    function updateFilter(data) {
        filter = data;
        localStorage.setItem(`${tablename}_filter`, JSON.stringify(data));
    }
</script>

<div class="toolbar">
  {#if Object.entries(filter).length > 0}
    FIlter:
    <span class="text-muted">
      {#each Object.entries(filter) as [key, value], index}
        {#if key === "IsRejected" || key === "IsRevised" || key === "IsFinished"}
          {key}: {value === "0" ? "No" : "Yes"}
        {:else}
          {key}: {value}
        {/if}
        {#if index < Object.entries(filter).length - 1}
          {" | "}
        {/if}
      {/each}
    </span>
  {:else}
    <span class="text-muted fw-bold">No Filter</span>
  {/if}
</div>
<table id="myTable" class="table table-striped" data-toggle="table"></table>
<ModalContainer id="filter" title="Filter Data Table" size="lg" >
    <ModalFilter data={data} tablename={tablename} updateFilter={updateFilter} initTable={initTable} />
</ModalContainer>