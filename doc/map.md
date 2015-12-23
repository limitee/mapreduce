#按key划分到另外一张表
```
select app_id, device_id, media, placement, count(*) into raw_tmp from raw group by app_id, device_id, media, placement;
```
