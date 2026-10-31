import { mount } from "svelte";
import "./tokens.css";
import Painel from "./lib/Painel.svelte";
import { nucleoTauri } from "./lib/nucleo";

mount(Painel, { target: document.getElementById("app")!, props: { nucleo: nucleoTauri } });
