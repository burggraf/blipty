<script lang="ts">
    import { cn } from "$lib/utils";
    import type { HTMLAttributes } from "svelte/elements";

    let { class: className = "", ...props } = $props<HTMLAttributes<HTMLDivElement>>();
</script>

<div class={cn("p-6 pt-0", className)} {...props}>
    {@render $$slots.default?.()}
</div>