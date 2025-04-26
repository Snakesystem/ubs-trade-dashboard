<script lang="ts">
    import { get, writable } from "svelte/store";
    import { closeModal, modals } from "../app";
    import { Modal } from "@sveltestrap/sveltestrap";
  
    export let id: string;
    export let title: string;
    export let size: string = "";
    export let centered: boolean = true;
  
    // Ambil state modal dari store
    let modal = writable(get(modals)[id]);
  
    // Subscribe ke perubahan store
    modals.subscribe((state) => {
      modal.set(state[id]);
    });

</script>

<Modal size={size} scrollable centered={centered} header={title} isOpen={$modal?.show} backdrop="static"
    toggle={() => closeModal(id)}>
    <div class="modal-body">
        <slot></slot>
    </div>
</Modal>
  