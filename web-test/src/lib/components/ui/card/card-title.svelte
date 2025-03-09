<script lang="ts">
    import { cn } from "$lib/utils";
    import type { HTMLHeadingAttributes } from "svelte/elements";

    let { class: className = "", ...props } = $props<HTMLHeadingAttributes>();
</script>

<h3
    class={cn("text-lg font-semibold leading-none tracking-tight", className)}
    {...props}
>
    {@render $$slots.default?.()}
</h3>