import StatusPage from './StatusPage.svelte';
import { mount } from "svelte";

const base_url = window.location.origin + window.location.pathname;

const target = document.getElementById('main');

const app = mount(StatusPage, {
  target, 
  anchor: target.firstChild,
  props: {
    base_url
  }
});

export default app;
