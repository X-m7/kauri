/** @format */

import { SET_STYLES } from "./types";

export const setStyles = (key, value) => ({
  type: SET_STYLES,
  key,
  value,
});
