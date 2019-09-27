/** @format */

import { SET_METADATA } from "./types";

export const setMetadata = (key, data) => ({
  type: SET_METADATA,
  key,
  data,
});
