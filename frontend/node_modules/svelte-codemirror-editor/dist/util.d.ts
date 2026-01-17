/**
 * Reduce calls to the passed function.
 *
 * @param func - Function to debounce.
 * @param threshold - The delay to avoid recalling the function.
 * @param execAsap - If true, the Function is called at the start of the threshold, otherwise the Function is called at the end of the threshold.
 */
export declare function debounce<T extends (...args: any[]) => any>(func: T, threshold: number, execAsap?: boolean): T;
