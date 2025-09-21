import { describe, expect, it, jest } from "@jest/globals";
import { DashTree, TreeNode } from "./dash-tree";

import {
  PayloadItem,
  TEST_SOURCE,
  TreeView,
  getPayloadItemChildren,
  getRandomInteger,
} from "@dashtree/testing";

describe("dash-tree.ts", () => {
  it("should keep tree data empty if no initial data is handled to constructor", () => {
    expect(new DashTree<PayloadItem>([], getPayloadItemChildren).nodes).toEqual(
      [],
    );
  });

  it("should keep tree data as is", () => {
    const { payloadItems, viewItems } = new TreeView(TEST_SOURCE);

    expect(
      new DashTree<PayloadItem>(payloadItems, getPayloadItemChildren).nodes.map(
        ({ payload }: TreeNode<PayloadItem>) => payload.value,
      ),
    ).toEqual(viewItems);
  });

  it("should call getChildren for each potential parent", () => {
    const { payloadItems, viewItems } = new TreeView(TEST_SOURCE);

    const observedGetChildren = jest.fn((payload: PayloadItem) => {
      return getPayloadItemChildren(payload);
    });

    new DashTree<PayloadItem>(payloadItems, observedGetChildren);

    expect(observedGetChildren).toHaveBeenCalledTimes(viewItems.length);
  });

  it("should find index of a desired node", () => {
    const { payloadItems, viewItems } = new TreeView(TEST_SOURCE);

    const targetIndex = getRandomInteger(0, viewItems.length);
    const targetView = viewItems.at(targetIndex);

    expect(
      new DashTree<PayloadItem>(payloadItems, getPayloadItemChildren).findIndex(
        ({ payload }: TreeNode<PayloadItem>) => payload.value === targetView,
      ),
    ).toBe(targetIndex);
  });

  it("should pull desired node by it's index", () => {
    const { payloadItems, viewItems } = new TreeView(TEST_SOURCE);

    const targetIndex = getRandomInteger(0, viewItems.length);
    const targetView = viewItems.at(targetIndex);

    expect(
      new DashTree<PayloadItem>(payloadItems, getPayloadItemChildren).get(
        targetIndex,
      ).payload.value,
    ).toBe(targetView);
  });

  it("should iterate over all nodes in the tree", () => {
    const { payloadItems, viewItems } = new TreeView(TEST_SOURCE);
    const exploredNodes: string[] = [];

    new DashTree<PayloadItem>(payloadItems, getPayloadItemChildren).forEach(
      (node: TreeNode<PayloadItem>) => {
        exploredNodes.push(node.payload.value);
      },
    );

    expect(exploredNodes).toEqual(viewItems);
  });
});
