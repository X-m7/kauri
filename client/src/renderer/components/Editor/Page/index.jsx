/** @format */

import "./styles.scss";
import { h } from "preact";

export default ({ styles, children }) => (
  <div class="page" style={styles}>
    {children}
  </div>
);
