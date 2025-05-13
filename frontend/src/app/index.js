// modalStore.ts
import { writable } from "svelte/store";
export const base_url = "http://localhost:8001/v1";

export const modals = writable({});

export const openModal = (/** @type {any} */ id) => {
  modals.update((prev) => ({ ...prev, [id]: { show: true } }));
};

export const closeModal = (/** @type {any} */ id) => {
  modals.update((prev) => ({ ...prev, [id]: { show: false } }));
};

export async function fetchColumns(tablename) {
  try {
    const response = await fetch(`${base_url}/data/header?tablename=${tablename}`);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Error fetching columns:", error);
  }
}