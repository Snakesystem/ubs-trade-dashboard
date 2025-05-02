<script>
// @ts-nocheck
  // Pastikan jQuery global tersedia
  import jQuery from 'jquery';
  window.$ = window.jQuery = jQuery;
  import Navigation from "./lib/Navigation.svelte";
  import * as bootstrap from 'bootstrap'
  import Router, { push, location } from 'svelte-spa-router';
  import menu from '../data/menu.json';
  import Home from "./routes/home.svelte";
  import Profile from "./routes/profile.svelte";
  import Dashboard from "./routes/dashboard.svelte";
  import Setting from "./routes/setting.svelte";
  import Help from "./routes/help.svelte";
  import Header from './lib/Header.svelte';
  import Login from './routes/login.svelte';

  let session = $state({
    result: false,
    message: "",
    data: [],
    error: ""
  });

  // Mapping string ke komponen
  const componentMap = {
    home: Home,
    profile: Profile,
    dashboard: Dashboard,
    settings: Setting,
    help: Help,
    login: Login
  };

  // Generate routes secara dinamis
  const routes = Object.fromEntries(
    menu.map(item => [`/${item.MenuID}`, componentMap[item.MenuID]])
  );

  // Tambahkan fallback route
  routes['*'] = Home;

  function navigateTo(path) {
    push('/' + path);
  }

  async function checkSession() {
    const response = await fetch(`https://snakesystem-web-api-tdam.shuttle.app/api/v1/auth/session`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
      }
    });
    const result = await response.json();
    if(result.result === false) {
      navigateTo('login');
    }
    session.set(result);
  }

  $effect(() => {

    checkSession();

    if ($location === '/' && session.result) {
      navigateTo('home');
    }
  })

</script>
<main>
  {#if $location !== '/login'}
    <Header/>
    <nav class="bottom-nav">
      {#each menu as item}
        {#if item.MenuID !== 'login'}
          <button class:active={$location === '/' + item.MenuID} onclick={() => navigateTo(item.MenuID)}>
            <i class="bi bi-{item.MenuIcon} me-2"></i>{item.MenuName}
          </button>
        {/if}
      {/each}
    </nav>
  {/if}

  <Router {routes} />
</main>

<style>
  main {
    padding-bottom: 4rem;
  }

  .bottom-nav {
    position: fixed;
    background-color: #444444;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.1);
    z-index: 999;
  }

  button {
    background: none;
    border: none;
    font-weight: bold;
    cursor: pointer;
    padding: 1rem;
  }

  .bottom-nav button.active {
    background-color: #212529;
    color: white;
    border-bottom: green 1px solid;
    border-left: green 1px solid;
    border-right: green 1px solid;
    border-radius: 0 0 10px 10px;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
    transition: background-color 0.3s ease-in-out;
  }
</style>