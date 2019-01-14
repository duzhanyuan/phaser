package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "phaser",
	Short: "Bloom's security scanner",
	Long:  "Bloom's security scanner. Visit https://bloom.sh for more information",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func Execute() error {
	return rootCmd.Execute()
}
