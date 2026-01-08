import { invoke } from '@tauri-apps/api/core'

class Logger {
  /**
   * Log an info message
   */
  async info(message: string, context?: string): Promise<void> {
    try {
      await invoke('log_from_frontend', { level: 'info', message, context })
    } catch (e) {
      console.debug('Failed to log info:', e)
    }
  }

  /**
   * Log an error message
   */
  async error(message: string, context?: string): Promise<void> {
    try {
      await invoke('log_from_frontend', { level: 'error', message, context })
      console.error(`[${context ?? 'error'}]`, message)
    } catch (e) {
      console.debug('Failed to log error:', e)
    }
  }

  /**
   * Log a user operation
   */
  async operation(action: string, details?: string): Promise<void> {
    const message = details ? `${action}: ${details}` : action
    await this.info(message, 'user-action')
  }

  /**
   * Get recent log lines
   */
  async getRecentLogs(lines = 50): Promise<string[]> {
    try {
      return await invoke<string[]>('get_recent_logs', { lines })
    } catch (e) {
      console.error('Failed to get recent logs:', e)
      return []
    }
  }

  /**
   * Get all logs
   */
  async getAllLogs(): Promise<string> {
    try {
      return await invoke<string>('get_all_logs')
    } catch (e) {
      console.error('Failed to get all logs:', e)
      return ''
    }
  }

  /**
   * Get the log file path
   */
  async getLogPath(): Promise<string> {
    try {
      return await invoke<string>('get_log_path')
    } catch (e) {
      console.error('Failed to get log path:', e)
      return ''
    }
  }

  /**
   * Open the log folder in system file manager
   */
  async openLogFolder(): Promise<void> {
    try {
      await invoke('open_log_folder')
    } catch (e) {
      console.error('Failed to open log folder:', e)
      throw e
    }
  }

  /**
   * Copy all logs to clipboard
   */
  async copyLogsToClipboard(): Promise<boolean> {
    try {
      const logs = await this.getAllLogs()
      if (logs) {
        await navigator.clipboard.writeText(logs)
        return true
      }
      return false
    } catch (e) {
      console.error('Failed to copy logs to clipboard:', e)
      return false
    }
  }
}

export const logger = new Logger()

// Convenience exports
export const logInfo = (message: string, context?: string) => logger.info(message, context)
export const logError = (message: string, context?: string) => logger.error(message, context)
export const logOperation = (action: string, details?: string) => logger.operation(action, details)
