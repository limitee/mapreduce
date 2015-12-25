#按key划分到另外一张表
```
select app_id, device_id, media, placement, count(*) into raw_tmp from raw group by app_id, device_id, media, placement;
```

#最细粒度
```
select app_id, device_id, media, placement, appv, source_type, subchannel, creative, keyword, type, region_id, mnc, net, brand, model, is_root, sdkv, count(*)::integer as event_count into raw_20151223 from raw group by app_id, device_id, media, placement, appv, source_type, subchannel, creative, keyword, type, region_id, mnc, net, brand, model, is_root, sdkv;
```
#type_count
```
CREATE FUNCTION type_count(integer, integer, integer) RETURNS integer AS '/data/pgsql/myfun', 'type_count' LANGUAGE C STRICT;
```

##type_count_ag
```
CREATE AGGREGATE type_count_ag (integer, integer)
(
    sfunc = type_count,
    stype = integer,
    initcond = '0'
);
```

#type_sum
```
CREATE FUNCTION type_sum(integer, integer, integer, integer) RETURNS integer AS '/data/pgsql/myfun', 'type_sum' LANGUAGE C STRICT;
```

##type_count_ag
```
CREATE AGGREGATE type_sum_ag (integer, integer, integer)
(
    sfunc = type_sum,
    stype = integer,
    initcond = '0'
);
```

#整合报告
```
select app_id, media, placement, appv, source_type, subchannel, creative, keyword, count(*) as device_count, sum(event_count) as event_count, type_count_ag(4, type) as reg_device_count, type_sum_ag(4, type, event_count) as reg_event_count into raw_20151223_zh from raw_20151223 group by app_id, media, placement, appv, source_type, subchannel, creative, keyword;
```

