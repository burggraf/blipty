<script lang="ts">
    import { cn } from "$lib/utils";
    import type { HTMLAttributes } from "svelte/elements";

    let { class: className = "", ...props } = $props<HTMLAttributes<HTMLDivElement>>();
</script>

<div
    class={cn(
        "bg-card text-card-foreground rounded-lg border shadow-sm",
        className
    )}
    {...props}
>
    {@render $$slots.default?.()}
</div>