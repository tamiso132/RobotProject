# Inventory System using Dobot Magican Robot

## Project Overview
An automatic inventory system that can handle orders without any interferences from outside. This system uses linux specific headers and is not crossplatform. 
It make use of the dobot magican for moving inventory items from/to orders
and sorting items coming into the storage.

## Bluetooth and UART
Linux specific, using linux headers for uart and bluez library for bluetooth connection.

## Protocol Development
There is Rust macros for adding new protcol commands

## Implemented Features
-Dobot Suction cup
-Dobot Conveyor belt
-Dobot Magican Movement
-Rasperry pi camera

## High Level Features
- Grid system for cubes (Inventory, Order placements)
- Ordersystem, (filling stock, do order, done with Order, sorting, order statistics)


