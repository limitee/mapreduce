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

/* by value */
PG_FUNCTION_INFO_V1(type_count);

Datum
type_count(PG_FUNCTION_ARGS)
{
    int32            state = PG_GETARG_INT32(0);
    int32            target_type = PG_GETARG_INT32(1);
    int32            raw_type = PG_GETARG_INT32(2);

    if(target_type == raw_type)
    {
        PG_RETURN_INT32(state + 1);
    }
    else
    {
        PG_RETURN_INT32(state);
    }
}

/* by value */
PG_FUNCTION_INFO_V1(type_sum);

Datum
type_sum(PG_FUNCTION_ARGS)
{
    int32            state = PG_GETARG_INT32(0);
    int32            target_type = PG_GETARG_INT32(1);
    int32            raw_type = PG_GETARG_INT32(2);
    int32            raw_count = PG_GETARG_INT32(3);

    if(target_type == raw_type)
    {
        PG_RETURN_INT32(state + raw_count);
    }
    else
    {
        PG_RETURN_INT32(state);
    }
}

/* by value */
PG_FUNCTION_INFO_V1(ctd_count);

Datum
ctd_count(PG_FUNCTION_ARGS)
{
    int32 state = PG_GETARG_INT32(0);
    int32 target_type = PG_GETARG_INT32(1);
    if(target_type == 2)
    {
        int32 created_at = PG_GETARG_INT32(2);
        int32 tkStamp = PG_GETARG_INT32(3);
        if(created_at - tkStamp < 20)
        {
            PG_RETURN_INT32(state + 1);
        }
        else
        {
            PG_RETURN_INT32(state);
        }
    }
    else
    {
        PG_RETURN_INT32(state);
    }
}

/* by value */
PG_FUNCTION_INFO_V1(repeat_count);

Datum
repeat_count(PG_FUNCTION_ARGS)
{
    int32 state = PG_GETARG_INT32(0);
    int32 target_type = PG_GETARG_INT32(1);
    if(target_type == 2)
    {
        int32 rtype = PG_GETARG_INT32(2);
        if(rtype != 2)
        {
            PG_RETURN_INT32(state + 1);
        }
        else
        {
            PG_RETURN_INT32(state);
        }
    }
    else
    {
        PG_RETURN_INT32(state);
    }
}