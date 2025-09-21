import { PayloadItem } from "./payload-item";

export class TreeView {
  readonly #viewItems: string[];

  public get viewItems(): string[] {
    return this.#viewItems.slice();
  }

  public get payloadItems(): PayloadItem[] {
    const itemsToProcess: PayloadItem[] = this.#viewItems.map(
      (value: string) => ({
        value,
        children: [],
      }),
    );

    itemsToProcess.forEach(
      (
        currentItem: PayloadItem,
        currentIndex: number,
        itemsSource: PayloadItem[],
      ) => {
        Object.defineProperty(
          currentItem,
          "children" satisfies keyof PayloadItem,
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

  constructor(viewItems: string[]) {
    this.#viewItems = viewItems;
  }

  static #getViewItemLevel(viewItem: string): number {
    return viewItem.length - viewItem.trimStart().length;
  }

  static #getDirectChildren(
    parentItem: PayloadItem,
    subsequentViewItems: PayloadItem[],
  ): PayloadItem[] {
    const parentLevel = TreeView.#getViewItemLevel(parentItem.value);
    const directChildLevel = parentLevel + 1;

    const directChildren: PayloadItem[] = [];

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

export const TEST_SOURCE = [
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
