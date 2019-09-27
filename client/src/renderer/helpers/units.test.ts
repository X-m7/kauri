/** @format */

import { convert } from "./units";

describe("convert", () => {
  it("should convert mm to px", () => {
    expect(convert(25.4, "mm")).toEqual(96);
  });

  it("should convert cm to px", () => {
    expect(convert(2.54, "cm")).toEqual(96);
  });
});
