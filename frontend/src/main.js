import { mount } from 'svelte'
import 'jquery'; // jQuery harus diimport dulu
import 'bootstrap-table/dist/bootstrap-table.min.css';
import 'bootstrap-table/dist/bootstrap-table.min.js';
import 'bootstrap-table/dist/extensions/auto-refresh/bootstrap-table-auto-refresh.min.js';
import 'bootstrap-icons/font/bootstrap-icons.css'
import 'bootstrap-table';
import 'bootstrap/dist/css/bootstrap.min.css'
import 'sweetalert2/dist/sweetalert2.min.css'
import './app.css'
import App from './App.svelte'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
