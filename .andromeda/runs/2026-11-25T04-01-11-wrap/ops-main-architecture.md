@row_add n=1 row=decision/virtual-node-role-is-accesskit-role area=established-decisions detector=none
name: A declared virtual node's role is AccessKit's Role
rule: A declared virtual node's role is AccessKit's Role itself, read by the tree as declared, with no closed role set or second role mapping of escher's own.
why: chosen over an escher role enum, a second role table to keep in step with the tree; the public declaration API names AccessKit's type in exchange
state: provisional
basis: fct-wkaf2evl
because: The fork answer F4 A (inputs#I2, a delegate's) chose AccessKit's own type over an escher enum mapped to it, a choice later readers of declared nodes must keep; the proposer's add stated no reason and fit read it as code.
@row_add n=2 row=decision/virtual-node-declaration-marks-nothing area=established-decisions detector=none
name: A declaration change marks and signals nothing yet
rule: A change of a canvas's declared virtual nodes writes no changed-set mark and sends no invalidation signal; both are left to the first reader that refreshes, «Virtual nodes as elements».
why: a mark or a signal now would amend the ratified changed-set write sites and marks for every document, while no reader in this chunk refreshes on it
state: provisional
basis: fct-wkaf2evl
because: The fork answers F1 B′ and F6 A (inputs#I2, a delegate's) leave the invalidation half and the mark to the first refreshing reader rather than amend two ratified rows now, a choice the next entry must take up; the proposer's add stated no reason and fit read it as code.
