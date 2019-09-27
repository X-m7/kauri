/** @format */

import { combineReducers } from "redux";

import cache from "./cache";
import commands from "./commands";
import document from "./document";
import metadata from "./metadata";
import styles from "./styles";
import pageStyles from "./pageStyles";

export default combineReducers({
  cache,
  commands,
  document,
  metadata,
  styles,
  pageStyles,
});
