"use client"

import * as React from "react";
import {ReactNode} from "react";
import {HeroUIProvider} from "@heroui/react";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";


// Create a client
const queryClient = new QueryClient()

export default function Providers(
    {children}:
    {children: ReactNode}
) {
    return (
        <QueryClientProvider client={queryClient}>
            <HeroUIProvider>
                {children}
            </HeroUIProvider>
        </QueryClientProvider>
    );
}