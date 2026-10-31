import { mount } from "svelte";
import "../../tokens.css";
import CompactPrototype from "./CompactPrototype.svelte";

// Throwaway browser entry: fixtures only, with no native core or mutations.
mount(CompactPrototype, { target: document.getElementById("app")! });
