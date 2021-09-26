import { Logger } from './types'
import consola from 'consola'

// Globally redirect all outputs to consola.
consola.wrapAll()

export const defaultLogger: Logger = consola
