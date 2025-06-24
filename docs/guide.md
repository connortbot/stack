# Guide
needs updates to be prettier :)

## Control
```bash
stack init
```
Initialize stack state.

```bash
stack checkout <stack>
```
Switches the current stack.
Use `-c / --create` flag to create a new stack.

```bash
stack status
```
Shows the current stack.

## Editing Stacks
```bash
stack delete <stack>
```
Delete a stack. `del`.

```bash
stack rm <index>
```
Remove branch from stack at index.

```bash
stack pop
```
Pop the stop of the stack.

```bash
stack insert <branch> <index>
```
Insert branch to a stack at an index.

```bash
stack push <branch>
```
Push branch to top of the stack.

```bash
stack label <index> "<label>"
```
Label a branch in the current stack.

## Updating Stacks
```bash
stack rebase <start_index> <end_index>
```
Iteratively rebases stack bottom-up, with optional range of indices.

```bash
stack pull
```
Runs `git pull` on each branch in the current stack.

```bash
stack list
```
Show all stacks.

