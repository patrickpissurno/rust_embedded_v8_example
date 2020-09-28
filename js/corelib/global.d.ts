/*
    This file provides typings for functions and constants
    that are globally available to JavaScript during the execution of
    this software.

    Notice that this is neither a Node.js environment nor a browser one.
    Don't expect things to work exactly as when working with them. Some
    common functions might be missing or having been replaced.
    
    For instance: 'console.log' doesn't exist. Instead: 'log' is provided.

    To make your IDE load this file, just copy & paste the following line
    to the top of your JavaScript files:

    /// <reference path="./corelib/global.d.ts"/>

    Don't forget to change the path appropriately!
*/

/**
 * Prints a message to the console
 * @param msg the message to be printed
 */
declare function log(msg: string): void;

/**
 * Loads a module from the specified path and returns it
 * @param path the absolute or relative path to the module
 */
declare function require(path: string): any;