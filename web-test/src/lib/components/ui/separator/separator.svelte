<script lang="ts">
    import { Separator as SeparatorPrimitive } from "bits-ui";
    import { cn } from "$lib/utils";
    import type { HTMLAttributes } from "svelte/elements";

    let {
        class: className = "",
        orientation = "horizontal",
        decorative = true,
        ...props
    } = $props<HTMLAttributes<HTMLDivElement> & {
        orientation?: "horizontal" | "vertical";
        decorative?: boolean;
    }>();
</script>

<SeparatorPrimitive.Root
    class={cn(
        "bg-border shrink-0",
        orientation === "horizontal" ? "h-[1px] w-full" : "h-full w-[1px]",
        className
    )}
    {orientation}
    {decorative}
    {...props}
/>
