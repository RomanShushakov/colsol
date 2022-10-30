## COLSOL

The **COLSOL** is an active column solver to obtain **LDL<sup>T</sup>** of a stiffness matrix or reduce and back-substitute the load vector.

### Input variables

**A** - stiffness matrix stored in compact form.

**NN** - number of equations.

**MAXA** - vector containing addresses of diagonal elements of stiffness matrix in **A**.

**V** - right-hand side load vector.

### Output variables

**A** - **D** and **L** factors of stiffness matrix.

**V** - displacement vector.

> Note: **COLSOL** can be used only for symmetric and positive definite stiffness matrices.

### Storage scheme

![storage_scheme](./docs/img/storage_scheme.svg)
