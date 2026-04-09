/**
 * Error handling utilities for user-friendly error messages
 */

export interface ErrorSolution {
  title: string
  description: string
  steps: string[]
  docLink?: string
}

/**
 * Get user-friendly error message and solution for common errors
 */
export function getErrorSolution(error: Error | string): ErrorSolution {
  const errorMsg = typeof error === 'string' ? error : error.message

  // Serial port errors
  if (errorMsg.includes('Permission denied') || errorMsg.includes('Access denied')) {
    return {
      title: '串口访问权限不足',
      description: '应用程序没有访问此串口的权限',
      steps: [
        '1. 关闭其他正在使用此串口的应用程序',
        '2. 以管理员身份运行应用程序',
        '3. 检查串口权限设置',
        '4. 在Linux上，将用户添加到dialout组：sudo usermod -a -G dialout $USER',
      ],
      docLink: '#troubleshooting-permission',
    }
  }

  if (errorMsg.includes('Port not found') || errorMsg.includes('does not exist')) {
    return {
      title: '串口不存在',
      description: '指定的串口设备未找到',
      steps: [
        '1. 点击"Refresh"重新扫描可用端口',
        '2. 检查串口设备是否正确连接',
        '3. 确认串口驱动程序已安装',
        '4. 在设备管理器中查看串口状态',
      ],
      docLink: '#troubleshooting-connection',
    }
  }

  if (errorMsg.includes('Busy') || errorMsg.includes('in use')) {
    return {
      title: '串口被占用',
      description: '此串口正被另一个应用程序使用',
      steps: [
        '1. 关闭所有串口调试工具（如PuTTY、CoolTerm等）',
        '2. 停止所有正在访问此串口的应用程序',
        '3. 重新拔插串口设备',
        '4. 重启计算机后重试',
      ],
      docLink: '#troubleshooting-busy',
    }
  }

  if (errorMsg.includes('Invalid configuration') || errorMsg.includes('Invalid parameter')) {
    return {
      title: '串口配置参数无效',
      description: '提供的串口配置参数不正确',
      steps: [
        '1. 检查波特率设置（常见值：9600, 115200）',
        '2. 确认数据位设置（通常为8）',
        '3. 验证停止位设置（通常为1）',
        '4. 检查校验位设置（通常为None）',
        '5. 尝试使用默认配置重新连接',
      ],
      docLink: '#troubleshooting-config',
    }
  }

  // Script execution errors
  if (errorMsg.includes('syntax error') || errorMsg.includes('parse error')) {
    return {
      title: '脚本语法错误',
      description: 'Lua脚本包含语法错误，无法执行',
      steps: [
        '1. 检查脚本中的括号、引号是否匹配',
        '2. 确认每行语句以分号或正确结尾',
        '3. 查看错误消息中指出的行号',
        '4. 使用Lua语法检查工具验证脚本',
        '5. 参考Lua脚本示例编写正确语法',
      ],
      docLink: '#scripting-syntax',
    }
  }

  if (errorMsg.includes('timeout') || errorMsg.includes('timed out')) {
    return {
      title: '操作超时',
      description: '操作耗时过长，已超时取消',
      steps: [
        '1. 检查串口设备是否正常响应',
        '2. 确认波特率等参数设置正确',
        '3. 减少脚本中的复杂操作',
        '4. 尝试增加超时时间设置',
        '5. 重启应用程序后重试',
      ],
      docLink: '#troubleshooting-timeout',
    }
  }

  // File operation errors
  if (errorMsg.includes('Failed to read file') || errorMsg.includes('Failed to write file')) {
    return {
      title: '文件操作失败',
      description: '无法读取或写入文件',
      steps: [
        '1. 确认文件路径正确',
        '2. 检查文件权限设置',
        '3. 确保磁盘空间充足',
        '4. 尝试使用其他文件名或路径',
        '5. 检查文件格式是否正确',
      ],
      docLink: '#troubleshooting-files',
    }
  }

  // Network/Connection errors
  if (errorMsg.includes('Connection refused') || errorMsg.includes('Network unreachable')) {
    return {
      title: '网络连接失败',
      description: '无法建立网络连接',
      steps: [
        '1. 检查网络连接是否正常',
        '2. 确认目标服务器地址和端口',
        '3. 检查防火墙设置',
        '4. 尝试ping目标地址',
        '5. 联系网络管理员',
      ],
      docLink: '#troubleshooting-network',
    }
  }

  // Default error
  return {
    title: '操作失败',
    description: errorMsg,
    steps: [
      '1. 重试操作',
      '2. 重启应用程序',
      '3. 查看控制台日志获取详细信息',
      '4. 检查系统资源使用情况',
      '5. 联系技术支持',
    ],
    docLink: '#troubleshooting-general',
  }
}

/**
 * Format error for display in UI
 */
export function formatError(error: Error | string): string {
  const errorMsg = typeof error === 'string' ? error : error.message

  // Extract common error patterns
  if (errorMsg.includes('Permission denied')) {
    return '权限不足：无法访问串口设备'
  }
  if (errorMsg.includes('Port not found')) {
    return '连接失败：找不到指定的串口'
  }
  if (errorMsg.includes('Busy')) {
    return '连接失败：串口正被其他程序使用'
  }
  if (errorMsg.includes('syntax error')) {
    return '执行失败：脚本包含语法错误'
  }
  if (errorMsg.includes('timeout')) {
    return '操作超时：设备响应时间过长'
  }

  return errorMsg
}

/**
 * Get error severity level
 */
export function getErrorSeverity(error: Error | string): 'info' | 'warning' | 'error' | 'critical' {
  const errorMsg = typeof error === 'string' ? error : error.message

  if (errorMsg.includes('Permission denied') || errorMsg.includes('Access denied')) {
    return 'critical'
  }
  if (errorMsg.includes('syntax error')) {
    return 'warning'
  }
  if (errorMsg.includes('timeout')) {
    return 'warning'
  }
  if (errorMsg.includes('Busy')) {
    return 'error'
  }

  return 'error'
}
