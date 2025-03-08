import { Dialog as DialogPrimitive } from "bits-ui";
import Root from "./dialog.svelte";
import Content from "./dialog-content.svelte";
import Description from "./dialog-description.svelte";
import Footer from "./dialog-footer.svelte";
import Header from "./dialog-header.svelte";
import Title from "./dialog-title.svelte";

export const Dialog = DialogPrimitive.Root;
export const DialogTrigger = DialogPrimitive.Trigger;
export const DialogClose = DialogPrimitive.Close;
export const DialogPortal = DialogPrimitive.Portal;
export const DialogOverlay = DialogPrimitive.Overlay;

export {
    Root,
    Content,
    Description,
    Footer,
    Header,
    Title,
    //
    Root as DialogRoot,
    Content as DialogContent,
    Description as DialogDescription,
    Footer as DialogFooter,
    Header as DialogHeader,
    Title as DialogTitle,
};