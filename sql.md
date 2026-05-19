-- Reliable Drivers: September 2024
-- Conditions: >= 5 deliveries, >= 90% COMPLETED rate for the month.
--
-- Half-open date range (>= start AND < next_month) lets the planner
-- use a B-tree index on attempt_timestamp directly. MONTH()/YEAR()
-- functions would force a full scan.

SELECT
    driver_id,
    COUNT(*)                                                             AS total_deliveries,
    ROUND(
        100.0
        * COUNT(*) FILTER (WHERE delivery_status = 'COMPLETED')
        / COUNT(*),
        2
    )                                                                    AS success_rate
FROM
    deliveries
WHERE
    attempt_timestamp >= '2024-09-01'
    AND attempt_timestamp  < '2024-10-01'
GROUP BY
    driver_id
HAVING
    COUNT(*) >= 5
    AND COUNT(*) FILTER (WHERE delivery_status = 'COMPLETED') * 100.0
        / COUNT(*) >= 90.0
ORDER BY
    success_rate DESC;

-- Recommended index (covers this query and any month-range variant):
-- CREATE INDEX idx_deliveries_ts_driver
--     ON deliveries (attempt_timestamp, driver_id, delivery_status);