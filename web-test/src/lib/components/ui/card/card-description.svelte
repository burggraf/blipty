<script lang="ts">
    import { cn } from "$lib/utils";
    import type { HTMLParagraphAttributes } from "svelte/elements";

    let { class: className = "", ...props } = $props<HTMLParagraphAttributes>();
</script>

<p
    class={cn("text-muted-foreground text-sm", className)}
    {...props}
>
    {@render $$slots.default?.()}
</p>