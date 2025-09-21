import { FlatTree } from "@dashtree/core";

type Predicate<T> = (node: Readonly<TreeNode<T>>) => boolean;
type Callback<T> = (node: Readonly<TreeNode<T>>) => void;

export interface TreeNode<T> {
  readonly payload: T;
  readonly level: number;
  readonly lastChildIndex: number;
  readonly hasChildren: boolean;
}

export class DashTree<T> {
  readonly #internal: FlatTree;

  public get nodes(): TreeNode<T>[] {
    return this.#internal.get_nodes();
  }

  constructor(rootPayloads: T[], getDirectChildren: (payload: T) => T[]) {
    this.#internal = new FlatTree(rootPayloads, getDirectChildren);
  }

  public findIndex(predicate: Predicate<T>): number | undefined {
    return this.#internal.find_index(predicate);
  }

  public get(index: number): Readonly<TreeNode<T>> {
    return this.#internal.get_by_index(index);
  }

  public forEach(callback: Callback<T>): void {
    this.#internal.for_each(callback);
  }

  public forEachChild(parentIndex: number, callback: Callback<T>): void {
    this.#internal.for_each_child(parentIndex, callback);
  }

  public forEachParent(targetIndex: number, callback: Callback<T>): void {
    this.#internal.for_each_parent(targetIndex, callback);
  }

  public getSubTreeByIndex(index: number): readonly Readonly<TreeNode<T>>[] {
    return this.#internal.get_sub_tree_by_index(index);
  }

  public replacePayload(targetIndex: number, payload: T): void {
    this.#internal.replace_payload(targetIndex, payload);
  }

  public delete(targetIndex: number): void {
    this.#internal.delete(targetIndex);
  }
}
