/* Gain access to Pg_magic_func() */

#include "postgres.h" // includes most of the basic stuff needed for interfacing with Postgres. This line needs to be included in every C-File that declares Postgres functions.
#include "fmgr.h" // needs to be included to make use of PG_GETARG_XXX and PG_RETURN_XXX macros. While this is valid for C we just need bindings and can write our own macros as you can see in lib.rs


PG_MODULE_MAGIC; // the “magic block” needed as of PostgreSQL 8.2 in one (and only one) of the module source files after including the header fmgr.h.
PG_FUNCTION_INFO_V1(base36_encode); // introduces the function to Postges as Version 1 Calling Convention, and is only needed if you want the function to interface with Postgres.
