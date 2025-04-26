<script>
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';
    import Sortable from 'sortablejs';
    const { filter, tablename } = $props()
  
    let charts = [
      {
        id: 'chart-1',
        title: 'Sales Report',
        type: 'bar',
        data: {
          labels: ['Jan', 'Feb', 'Mar', 'Apr'],
          datasets: [
            {
              label: 'Sales',
              data: [100, 200, 150, 250],
              backgroundColor: '#60a5fa'
            }
          ]
        }
      },
      {
        id: 'chart-2',
        title: 'Revenue Report',
        type: 'line',
        data: {
          labels: ['Q1', 'Q2', 'Q3', 'Q4'],
          datasets: [
            {
              label: 'Revenue',
              data: [400, 300, 500, 450],
              borderColor: '#34d399',
              fill: false
            }
          ]
        }
      },
      {
        id: 'chart-3',
        title: 'Customer Growth',
        type: 'doughnut',
        data: {
          labels: ['New', 'Returning'],
          datasets: [
            {
              label: 'Customers',
              data: [120, 80],
              backgroundColor: ['#fbbf24', '#f87171']
            }
          ]
        }
      }
    ];
  
    let sortableContainer;
  
    onMount(() => {
      charts.forEach(chart => {
        const canvas = document.getElementById(chart.id);
        new Chart(canvas, {
          type: chart.type,
          data: chart.data,
          options: {
            responsive: true,
            maintainAspectRatio: false
          }
        });
      });
  
      Sortable.create(sortableContainer, {
        animation: 200,
        ghostClass: 'drag-ghost'
      });
    });
  </script>
  
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
  
  <div bind:this={sortableContainer} class="dashboard-container">
    {#each charts as chart}
      <div class="card-item">
        <h6 class="fw-bold text-center mb-2">{chart.title}</h6>
        <canvas id={chart.id}></canvas>
      </div>
    {/each}
  </div>
  