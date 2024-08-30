package tree_sitter_nasin_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_nasin "github.com/tree-sitter/tree-sitter-nasin/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_nasin.Language())
	if language == nil {
		t.Errorf("Error loading Nasin grammar")
	}
}
