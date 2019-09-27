/** @format */

import { SET_PAGE_STYLES } from "./types";

export const setPageStyles = (key, value) => ({
  type: SET_PAGE_STYLES,
  key,
  value,
});
