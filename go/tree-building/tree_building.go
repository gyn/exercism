package tree

import (
	"errors"
	"fmt"
	"sort"
)

// Record keeps a ID and its parent ID
type Record struct {
	ID, Parent int
}

// Node keeps a ID and its children's ID
type Node struct {
	ID       int
	Children []*Node
}

// Build builds a Node from records
func Build(records []Record) (*Node, error) {
	if len(records) == 0 {
		return nil, nil
	}

	err := checkRecords(records)
	if err != nil {
		return nil, err
	}

	return buildTree(records), nil
}

func checkRecords(records []Record) error {
	//
	// sort slice by ID
	//
	sort.Slice(records, func(i, j int) bool {
		if records[i].ID > records[j].ID {
			return true
		} else if records[i].ID == records[j].ID {
			return records[i].Parent < records[j].Parent
		} else {
			return false
		}
	})

	//
	// check root node
	//
	last := len(records) - 1
	if records[last].ID != 0 || records[last].Parent != 0 {
		return errors.New("invalid root node")
	}

	// ignore the last node
	for i := 0; i < last; i++ {
		//
		// ID is unique and between [0, len(records))
		//
		if records[i].ID != last-i {
			return fmt.Errorf("invalid ID for node %v", records[i])
		}

		//
		// all records except the record for root node should have a parent ID lower than their own ID
		//
		if records[i].Parent >= records[i].ID {
			return fmt.Errorf("invalid node %v", records[i])
		}
	}

	return nil
}

func buildTree(records []Record) *Node {
	//
	// if there is only one record, it must be root node
	//
	if len(records) == 1 {
		return &Node{0, nil}
	}

	//
	// ignore the root node
	//
	records = records[:len(records)-1]

	return buildTreeByNodes(records)
}

func buildTreeByNodes(records []Record) *Node {
	//
	// sort slice by Parent
	//
	sort.SliceStable(records, func(i, j int) bool {
		if records[i].Parent < records[j].Parent {
			return true
		} else if records[i].Parent == records[j].Parent {
			return records[i].ID > records[j].ID
		} else {
			return false
		}
	})

	//
	// add extra 1 for root node
	//
	slice := make([]*Node, len(records)+1)

	for i := len(records) - 1; i >= 0; i-- {
		child := records[i].ID
		parent := records[i].Parent

		if slice[child] == nil {
			slice[child] = &Node{child, nil}
		}

		if slice[parent] == nil {
			slice[parent] = &Node{parent, []*Node{slice[child]}}
		} else {
			slice[parent].Children = append(slice[parent].Children, slice[child])
		}
	}

	return slice[0]
}
