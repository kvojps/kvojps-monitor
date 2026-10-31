import { mount } from "svelte";
import "./tokens.css";
import Panel from "./lib/Panel.svelte";
import { tauriCore } from "./lib/core";

mount(Panel, { target: document.getElementById("app")!, props: { core: tauriCore } });
