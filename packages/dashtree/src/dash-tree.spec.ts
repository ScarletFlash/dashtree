import { describe, expect, it, jest } from "@jest/globals";
import { DashTree, TreeNode } from "./dash-tree";

interface PayloadSource {
  readonly value: string;
  readonly children: PayloadSource[];
}

function getChildren({ children }: PayloadSource): PayloadSource[] {
  return children;
}

function getRandomInteger(min = 0, max = 100): number {
  if (min > max) {
    throw new Error(
      `[min: ${min}] must be less than or equal to [max: ${max}]`,
    );
  }

  const minSpan: number = min === -Infinity ? 0 : min;
  const maxSpan: number = max === Infinity ? 0 : max - minSpan;

  const randomValue: number = Math.random();
  return Math.floor(randomValue * (maxSpan + 1)) + minSpan;
}

class TreeView {
  readonly #view: string[];

  public get view(): string[] {
    return this.#view.slice();
  }

  public get payloadSources(): PayloadSource[] {
    const itemsToProcess: PayloadSource[] = this.#view.map(
      (currentItem: string) => ({
        value: currentItem,
        children: [],
      }),
    );

    itemsToProcess.forEach(
      (
        currentItem: PayloadSource,
        currentIndex: number,
        itemsSource: PayloadSource[],
      ) => {
        Object.defineProperty(
          currentItem,
          "children" satisfies keyof PayloadSource,
          {
            value: TreeView.#getDirectChildren(
              currentItem,
              itemsSource.slice(currentIndex + 1),
            ),
          },
        );
      },
    );

    return itemsToProcess.slice(0, 1);
  }

  constructor(view: string[]) {
    this.#view = view;
  }

  static #getViewItemLevel(viewItem: string): number {
    return viewItem.length - viewItem.trimStart().length;
  }

  static #getDirectChildren(
    parentItem: PayloadSource,
    subsequentViewItems: PayloadSource[],
  ): PayloadSource[] {
    const parentLevel = TreeView.#getViewItemLevel(parentItem.value);
    const directChildLevel = parentLevel + 1;

    const directChildren: PayloadSource[] = [];

    for (const currentItem of subsequentViewItems) {
      const currentItemLevel = TreeView.#getViewItemLevel(currentItem.value);

      if (currentItemLevel <= parentLevel) {
        break;
      }

      if (currentItemLevel === directChildLevel) {
        directChildren.push(currentItem);
      }
    }

    return directChildren;
  }
}

const TEST_SOURCE = [
  "1",
  " 2",
  " 3",
  " 4",
  "  5",
  "  6",
  "   7",
  "   8",
  "    9",
  "    10",
  "    11",
  "     12",
  "      13",
  "       14",
  "        15",
  "   16",
  "  17",
  " 18",
  "  19",
  "   20",
  "    21",
  "     21",
];

describe("dash-tree.ts", () => {
  it("should keep tree data empty if no initial data is handled to constructor", () => {
    expect(new DashTree<PayloadSource>([], getChildren).nodes).toEqual([]);
  });

  it("should keep tree data as is", () => {
    const { payloadSources, view } = new TreeView(TEST_SOURCE);

    expect(
      new DashTree<PayloadSource>(payloadSources, getChildren).nodes.map(
        ({ payload }: TreeNode<PayloadSource>) => payload.value,
      ),
    ).toEqual(view);
  });

  it("should call getChildren for each potential parent", () => {
    const { payloadSources, view } = new TreeView(TEST_SOURCE);

    const observedGetChildren = jest.fn((payload: PayloadSource) => {
      return getChildren(payload);
    });

    new DashTree<PayloadSource>(payloadSources, observedGetChildren);

    expect(observedGetChildren).toHaveBeenCalledTimes(view.length);
  });

  it("should find index of a desired node", () => {
    const { payloadSources, view } = new TreeView(TEST_SOURCE);

    const targetIndex = getRandomInteger(0, view.length);
    const targetView = view.at(targetIndex);

    expect(
      new DashTree<PayloadSource>(payloadSources, getChildren).findIndex(
        ({ payload }: TreeNode<PayloadSource>) => payload.value === targetView,
      ),
    ).toBe(targetIndex);
  });

  it("should pull desired node by it's index", () => {
    const { payloadSources, view } = new TreeView(TEST_SOURCE);

    const targetIndex = getRandomInteger(0, view.length);
    const targetView = view.at(targetIndex);

    expect(
      new DashTree<PayloadSource>(payloadSources, getChildren).get(targetIndex)
        .payload.value,
    ).toBe(targetView);
  });

  it("should iterate over all nodes in the tree", () => {
    const { payloadSources, view } = new TreeView(TEST_SOURCE);
    const exploredNodes: string[] = [];

    new DashTree<PayloadSource>(payloadSources, getChildren).forEach(
      (node: TreeNode<PayloadSource>) => {
        exploredNodes.push(node.payload.value);
      },
    );

    expect(exploredNodes).toEqual(view);
  });
});
