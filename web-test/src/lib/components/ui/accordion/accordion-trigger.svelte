<script lang="ts">
    import { cn } from "$lib/utils";
    import { Accordion as AccordionPrimitive } from "bits-ui";
    import { ChevronDown } from "lucide-svelte";

    let { class: className = "", ...props } = $props<AccordionPrimitive.TriggerProps>();
</script>

<AccordionPrimitive.Header class="flex">
    <AccordionPrimitive.Trigger
        class={cn(
            "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180",
            className
        )}
        {...props}
    >
        {@render $$slots.default?.()}
        <ChevronDown class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200" />
    </AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
