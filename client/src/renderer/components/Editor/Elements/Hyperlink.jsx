/** @format */

import { h } from "preact";
import { Element } from "./index";

export const Hyperlink = props => (
  <Element tag="a" attributes={{ href: props.href }} {...props} />
);
