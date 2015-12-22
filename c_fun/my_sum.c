#include "postgres.h"
#include <string.h>
#include "fmgr.h"
#include "utils/geo_decls.h"
#include "executor/executor.h"  /* for GetAttributeByName() */
#include "funcapi.h"

/* by value */
PG_FUNCTION_INFO_V1(my_sum);

Datum
my_sum(PG_FUNCTION_ARGS)
{
    int32            state = PG_GETARG_INT32(0);
    int32            cur = PG_GETARG_INT32(1);
    PG_RETURN_INT32(state + cur);
}
