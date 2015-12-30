#索引
```
CREATE INDEX raw_date_idx ON raw (date);
```

#最细粒度
```
select app_id, device_id, media, placement, appv, source_type, subchannel, creative, keyword, type, region_id, mnc, net, brand, model, is_jailbreak, is_root, sdkv, count(*)::integer as event_count, type_count_ag(1, extra) as recalls, sum(purchase_value)::integer as purchase_value, ctd_count_ag(type, created_at, tkstamp)::integer as ctd, repeat_count_ag(type, rtype)::integer as repeat into raw_20151223 from raw  where date=20151223 group by app_id, device_id, media, placement, appv, source_type, subchannel, creative, keyword, type, region_id, mnc, net, brand, model, is_jailbreak, is_root, sdkv;
```

##激活设备排重
```
alter table raw_20151223 add column is_active_device integer default 0;
update raw_20151223 set is_active_device=1 where type=2 and event_count - repeat > 0;
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
>注意激活设备是去重之后的设备数，使用raw_20151223报表的is_active_device字段
```
select app_id, media, placement, appv, source_type, subchannel, creative, keyword, count(*) as device_count, sum(event_count) as event_count, sum(recalls) as recalls, sum(is_active_device) as activce_device_count, type_sum_ag(2, type, event_count) as active_event_count, type_count_ag(3, type) as start_device_count, type_sum_ag(3, type, event_count) as start_event_count, type_count_ag(4, type) as reg_device_count, type_sum_ag(4, type, event_count) as reg_event_count, type_count_ag(5, type) as login_device_count, type_sum_ag(5, type, event_count) as login_event_count, type_count_ag(7, type) as ucustom1s, type_sum_ag(7, type, event_count) as custom1s, type_count_ag(8, type) as ucustom2s, type_sum_ag(8, type, event_count) as custom2s, type_count_ag(9, type) as ucustom3s, type_sum_ag(9, type, event_count) as custom3s, type_count_ag(10, type) as ucustom4s, type_sum_ag(10, type, event_count) as custom4s, type_count_ag(11, type) as ucustom5s, type_sum_ag(11, type, event_count) as custom5s into raw_20151223_zh from raw_20151223 group by app_id, media, placement, appv, source_type, subchannel, creative, keyword;
```

#当天的激活设备
```
select app_id, device_id, media, placement into raw_20151223_active_device from raw_20151223  where type=2 and is_active_device=1 group by app_id, device_id, media, placement;
```

#当天的启动设备数
```
select app_id, device_id, media, placement into raw_20151223_start_device from raw_20151223 where type=3 group by app_id, device_id, media, placement;
```

#计算23日激活设备在24日的留存

##修改表结构
```
alter table raw_20151223_active_device add column start_20151224 integer default 0;
```

##23日激活，24日启动，共有设备
```
select ad.media, ad.placement, ad.device_id from raw_20151223_active_device as ad inner join raw_20151224_start_device as sd on ad.device_id=sd.device_id and ad.media = sd.media and ad.placement = sd.placement order by ad.media, ad.placement, ad.device_id;
```

##纪录设备信息
>mysql中inner join的语法如下
```
update raw_20151223_active_device set start_20151224=1 from raw_20151223_active_device as ad inner join raw_20151224_start_device as sd on ad.device_id=sd.device_id and ad.media = sd.media and ad.placement = sd.placement;
```

>pg中的联表更新如下
```
update raw_20151223_active_device as ad set start_20151224=1 from raw_20151224_start_device as sd where ad.device_id=sd.device_id and ad.media = sd.media and ad.placement = sd.placement;
```

##出结果(23号在24号的留存)
```
select media, placement, count(*), type_count_ag(1, start_20151224) from raw_20151223_active_device group by media, placement;
```

#活跃数

##当天
```
select media, placement, count(*) from (select media, placement, device_id from raw where date=20151223 group by media, placement, device_id) as temp group by media, placement;
```

#设备和地域

##地域
```
select app_id, media, placement, region_id, appv, source_type, type_count_ag(2, type) as activce_device_count, type_sum_ag(2, type, event_count) as active_event_count, type_count_ag(3, type) as start_device_count, type_sum_ag(3, type, event_count) as start_event_count, type_count_ag(4, type) as reg_device_count, type_sum_ag(4, type, event_count) as reg_event_count, type_count_ag(5, type) as login_device_count, type_sum_ag(5, type, event_count) as login_event_count, type_count_ag(7, type) as ucustom1s, type_sum_ag(7, type, event_count) as custom1s, type_count_ag(8, type) as ucustom2s, type_sum_ag(8, type, event_count) as custom2s, type_count_ag(9, type) as ucustom3s, type_sum_ag(9, type, event_count) as custom3s, type_count_ag(10, type) as ucustom4s, type_sum_ag(10, type, event_count) as custom4s, type_count_ag(11, type) as ucustom5s, type_sum_ag(11, type, event_count) as custom5s into raw_20151223_region from raw_20151223 group by app_id, media, placement, region_id, appv, source_type;
```

##设备
```
select app_id, media, placement, appv, mnc, net, brand, model, is_jailbreak, is_root, sdkv,source_type, type_count_ag(2, type) as activce_device_count, type_sum_ag(2, type, event_count) as active_event_count, type_count_ag(3, type) as start_device_count, type_sum_ag(3, type, event_count) as start_event_count, type_count_ag(4, type) as reg_device_count, type_sum_ag(4, type, event_count) as reg_event_count, type_count_ag(5, type) as login_device_count, type_sum_ag(5, type, event_count) as login_event_count, type_count_ag(7, type) as ucustom1s, type_sum_ag(7, type, event_count) as custom1s, type_count_ag(8, type) as ucustom2s, type_sum_ag(8, type, event_count) as custom2s, type_count_ag(9, type) as ucustom3s, type_sum_ag(9, type, event_count) as custom3s, type_count_ag(10, type) as ucustom4s, type_sum_ag(10, type, event_count) as custom4s, type_count_ag(11, type) as ucustom5s, type_sum_ag(11, type, event_count) as custom5s into raw_20151223_device from raw_20151223 group by app_id, media, placement, appv, mnc, net, brand, model, is_jailbreak, is_root, sdkv, source_type;
```

#订单
```
select app_id, media, placement, type_sum_ag(12, type, event_count) as order_count, type_sum_ag(12, type, purchase_value) as order_value, type_sum_ag(13, type, event_count) as purchase_count, type_sum_ag(13, type, purchase_value) as purchase_value into raw_20151223_order from raw_20151223 group by app_id, media, placement;
```

#反作弊

##异常转化

###加载函数
```
CREATE FUNCTION ctd_count(integer, integer, integer, integer) RETURNS integer AS '/data/pgsql/myfun', 'ctd_count' LANGUAGE C STRICT;
```

###生成聚合函数
事件类型，纪录创建时间，点击时间
```
CREATE AGGREGATE ctd_count_ag (integer, integer, integer)
(
    sfunc = ctd_count,
    stype = integer,
    initcond = '0'
);
```

###生成报表
```
select app_id, media, placement, count(*) as device_count, sum(ctd) as event_count into raw_20151223_cheat_ctd from raw_20151223 where type=2 and ctd>0 and repeat=0 group by app_id, media, placement;
```

##重复

###加载函数
```
CREATE FUNCTION repeat_count(integer, integer, integer) RETURNS integer AS '/data/pgsql/myfun', 'repeat_count' LANGUAGE C STRICT;
```

###生成聚合函数
>事件类型，rtype
```
CREATE AGGREGATE repeat_count_ag (integer, integer)
(
    sfunc = repeat_count,
    stype = integer,
    initcond = '0'
);
```

###生成报表
```
select app_id, media, placement, count(*) as device_count, sum(repeat) as event_count into raw_20151223_cheat_repeat from raw_20151223 where type=2 and repeat>0 group by app_id, media, placement;
```