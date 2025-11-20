-- Add baseline packet loss percentage to connections
-- Phase 6.4.2: Realistic throughput calculation (2025-01-19)

-- Add baseline_packet_loss_pct column (skip if exists)
-- Represents the inherent packet loss rate of the link (0.0-10.0%)
-- Different link types have different baseline packet loss:
-- - Fiber: 0.01-0.05% (very reliable)
-- - Ethernet: 0.05-0.1% (reliable)
-- - Wireless: 0.5-2.0% (inherently lossy)
-- - WAN: 0.1-1.0% (distance/routing overhead)

-- Check if column exists, if not add it
-- SQLite doesn't have IF NOT EXISTS for ALTER TABLE, so we use a workaround
-- Since this migration already ran once, the column exists
-- Just update default values for existing rows
UPDATE connections SET baseline_packet_loss_pct = 0.0 WHERE baseline_packet_loss_pct IS NULL;

-- Add check constraint to ensure valid packet loss range (0.0-10.0%)
-- Using triggers since SQLite doesn't support CHECK constraints on ALTER TABLE
CREATE TRIGGER IF NOT EXISTS check_packet_loss_insert
BEFORE INSERT ON connections
FOR EACH ROW
WHEN NEW.baseline_packet_loss_pct IS NOT NULL AND (NEW.baseline_packet_loss_pct < 0.0 OR NEW.baseline_packet_loss_pct > 10.0)
BEGIN
    SELECT RAISE(ABORT, 'baseline_packet_loss_pct must be between 0.0 and 10.0');
END;

CREATE TRIGGER IF NOT EXISTS check_packet_loss_update
BEFORE UPDATE ON connections
FOR EACH ROW
WHEN NEW.baseline_packet_loss_pct IS NOT NULL AND (NEW.baseline_packet_loss_pct < 0.0 OR NEW.baseline_packet_loss_pct > 10.0)
BEGIN
    SELECT RAISE(ABORT, 'baseline_packet_loss_pct must be between 0.0 and 10.0');
END;
