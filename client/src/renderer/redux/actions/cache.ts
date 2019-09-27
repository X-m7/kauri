/** @format */

import { CACHE_NODE, CACHE_WORKING_AREA } from "./types";

/**
 * Cache a node's height.
 * @param id Node id.
 * @param height Node rendered height.
 */
export const cacheNode = (id: number, attrs: object) => ({
  type: CACHE_NODE,
  payload: { id, ...attrs },
});

/**
 * Cache a page's working area.
 * @param width Working area width.
 * @param height Working area height.
 */
export const cacheWorkingArea = (width: number, height: number) => ({
  type: CACHE_WORKING_AREA,
  payload: { width, height },
});
