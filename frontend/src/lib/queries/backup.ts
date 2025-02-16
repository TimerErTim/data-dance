import config from "@/lib/config";
import {useMutation, useQueryClient} from "@tanstack/react-query";

export function useStartBackupMutation(
    params?: {
        onSuccess?: () => void,
        onError?: () => void
    }
) {
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: async () => {
            const response = await fetch(config.host + '/api/jobs/incremental_backup', {
                method: 'POST',
            });
            return response.status
        },
        onSuccess: async () => {
            await queryClient.invalidateQueries({queryKey: ['currentJobs']})
            params?.onSuccess?.()
        },
        onError: async () => {
            params?.onError?.()
        }
    })
}
