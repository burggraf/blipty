// Extend the Navigator interface to include deviceMemory
declare global {
  interface Navigator {
    readonly deviceMemory?: number;
  }
}
