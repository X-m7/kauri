/** @format */

import createShortcut from "./shortcuts.js";

describe("createShortcut", () => {
  it("should turn a string to a shortcut object", () => {
    expect(createShortcut("control+c")).toEqual({
      id: 1,
      isAlt: false,
      isCtrl: true,
      isMeta: false,
      isShift: false,
      key: "c",
    });
  });
  it("Should ignore order of modifiers, as long as it ends in the letter", () => {
    expect(createShortcut("alt+control+meta+shift+w")).toEqual({
      id: 2,
      isAlt: true,
      isCtrl: true,
      isMeta: true,
      isShift: true,
      key: "w",
    });
  });
  it("Should ignore case", () => {
    expect(createShortcut("MetA+ALt+coNtRol+A")).toEqual({
      id: 3,
      isAlt: true,
      isCtrl: true,
      isMeta: true,
      isShift: false,
      key: "a",
    });
  });
  it("Should throw an error on empty input", () => {
    expect(() => {
      createShortcut("");
    }).toThrow("Cannot create an empty shortcut");
  });
  it("Should accept just a key", () => {
    expect(createShortcut("c")).toEqual({
      id: 4,
      isAlt: false,
      isCtrl: false,
      isMeta: false,
      isShift: false,
      key: "c",
    });
  });
});
