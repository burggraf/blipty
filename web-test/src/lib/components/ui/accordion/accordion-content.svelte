<script lang="ts">
    import { cn } from "$lib/utils";
    import { Accordion as AccordionPrimitive } from "bits-ui";
    import { slide } from "svelte/transition";
    import type { TransitionConfig } from "svelte/transition";

    type $$Props = AccordionPrimitive.ContentProps & {
        transition?: (node: Element, options: any) => TransitionConfig;
        transitionConfig?: Record<string, any>;
    };

    let {
        class: className = "",
        transition = slide,
        transitionConfig = {
            duration: 200
        },
        ...props
    } = $props<$$Props>();
</script>

<AccordionPrimitive.Content
    class={cn("overflow-hidden text-sm", className)}
    transition={transition}
    transitionConfig={transitionConfig}
    {...props}
>
    <div class="pb-4 pt-0">
        {@render $$slots.default?.()}
    </div>
</AccordionPrimitive.Content>
